use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Default)]
pub struct LineCountReport {
    pub input_lines: usize,
    pub output_lines: usize,
    pub skipped_items: Vec<SkippedItem>,
    pub processed_items: usize,
    pub error_items: Vec<ErrorItem>,
}

#[derive(Debug, Clone)]
pub struct SkippedItem {
    pub item_type: String,
    pub reason: String,
    pub content_preview: String,
}

#[derive(Debug, Clone)]
pub struct ErrorItem {
    pub item_type: String,
    pub error_message: String,
    pub content_preview: String,
}

impl LineCountReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_input_lines(&mut self, lines: usize) {
        self.input_lines += lines;
    }

    pub fn add_output_lines(&mut self, lines: usize) {
        self.output_lines += lines;
    }

    pub fn add_skipped_item(&mut self, item_type: String, reason: String, content: &str) {
        let preview = content.lines().take(3).collect::<Vec<_>>().join(" ");
        self.skipped_items.push(SkippedItem {
            item_type,
            reason,
            content_preview: preview.chars().take(100).collect(),
        });
    }

    pub fn add_error_item(&mut self, item_type: String, error: String, content: &str) {
        let preview = content.lines().take(3).collect::<Vec<_>>().join(" ");
        self.error_items.push(ErrorItem {
            item_type,
            error_message: error,
            content_preview: preview.chars().take(100).collect(),
        });
    }

    pub fn add_processed_item(&mut self) {
        self.processed_items += 1;
    }

    pub fn print_summary(&self) {
        println!("\n📊 LINE COUNT REPORT");
        println!("==================");
        println!("📥 Input lines:     {}", self.input_lines);
        println!("📤 Output lines:    {}", self.output_lines);
        println!("✅ Processed items: {}", self.processed_items);
        println!("⚠️  Skipped items:   {}", self.skipped_items.len());
        println!("❌ Error items:     {}", self.error_items.len());
        
        let coverage = if self.input_lines > 0 {
            (self.output_lines as f64 / self.input_lines as f64) * 100.0
        } else {
            0.0
        };
        println!("📈 Coverage:        {:.1}%", coverage);

        if !self.skipped_items.is_empty() {
            println!("\n⚠️  SKIPPED ITEMS:");
            for (i, item) in self.skipped_items.iter().enumerate() {
                println!("  {}. {} - {} | {}", i+1, item.item_type, item.reason, item.content_preview);
            }
        }

        if !self.error_items.is_empty() {
            println!("\n❌ ERROR ITEMS:");
            for (i, item) in self.error_items.iter().enumerate() {
                println!("  {}. {} - {} | {}", i+1, item.item_type, item.error_message, item.content_preview);
            }
        }
    }
}

pub fn count_lines_in_file(path: &Path) -> Result<usize> {
    let content = std::fs::read_to_string(path)?;
    Ok(content.lines().count())
}

pub fn count_lines_in_string(content: &str) -> usize {
    content.lines().count()
}
