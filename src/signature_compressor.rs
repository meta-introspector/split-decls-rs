use std::collections::HashMap;
use anyhow::Result;

/// Signature compression system using prime numbers and emojis
pub struct SignatureCompressor {
    /// Prime number assignments (2 = most common, higher primes = rarer)
    pub prime_assignments: HashMap<String, u64>,
    /// Emoji assignments for visual representation
    pub emoji_assignments: HashMap<String, String>,
    /// Frequency tracking for optimal prime assignment
    pub frequency_map: HashMap<String, u64>,
    /// Next available prime
    pub next_prime: u64,
}

/// Macro binding signature - the string of bindings needed for a declaration
#[derive(Debug, Clone)]
pub struct MacroBindingSignature {
    pub bindings: Vec<String>,
    pub signature_string: String,
    pub prime_key: u64,
    pub emoji_key: String,
    pub frequency: u64,
}

impl SignatureCompressor {
    pub fn new() -> Self {
        Self {
            prime_assignments: HashMap::new(),
            emoji_assignments: HashMap::new(),
            frequency_map: HashMap::new(),
            next_prime: 2,
        }
    }

    /// Extract macro binding signature from a declaration
    pub fn extract_signature(&mut self, decl_code: &str) -> Result<MacroBindingSignature> {
        let bindings = self.parse_macro_bindings(decl_code)?;
        let signature_string = bindings.join("|");
        
        // Update frequency
        let frequency = self.frequency_map.entry(signature_string.clone())
            .and_modify(|f| *f += 1)
            .or_insert(1);
        
        // Get or assign prime and emoji
        let prime_key = self.get_or_assign_prime(&signature_string);
        let emoji_key = self.get_or_assign_emoji(&signature_string);
        
        Ok(MacroBindingSignature {
            bindings,
            signature_string,
            prime_key,
            emoji_key,
            frequency: *frequency,
        })
    }

    /// Parse macro bindings from declaration code
    fn parse_macro_bindings(&self, code: &str) -> Result<Vec<String>> {
        let mut bindings = Vec::new();
        
        // Look for common macro binding patterns
        if code.contains("prelude!") {
            bindings.push("prelude".to_string());
        }
        if code.contains("#[decl_") {
            bindings.push("decl_attr".to_string());
        }
        if code.contains("use ") {
            bindings.push("use_stmt".to_string());
        }
        if code.contains("pub fn") {
            bindings.push("pub_fn".to_string());
        }
        if code.contains("pub struct") {
            bindings.push("pub_struct".to_string());
        }
        if code.contains("pub enum") {
            bindings.push("pub_enum".to_string());
        }
        if code.contains("impl ") {
            bindings.push("impl_block".to_string());
        }
        if code.contains("trait ") {
            bindings.push("trait_def".to_string());
        }
        if code.contains("macro_rules!") {
            bindings.push("macro_def".to_string());
        }
        if code.contains("derive(") {
            bindings.push("derive_attr".to_string());
        }
        
        // Sort for consistent signatures
        bindings.sort();
        bindings.dedup();
        
        Ok(bindings)
    }

    /// Get or assign prime number based on frequency (most common = 2)
    fn get_or_assign_prime(&mut self, signature: &str) -> u64 {
        if let Some(&prime) = self.prime_assignments.get(signature) {
            return prime;
        }
        
        let prime = self.next_prime;
        self.prime_assignments.insert(signature.to_string(), prime);
        self.next_prime = self.next_prime_number(self.next_prime);
        prime
    }

    /// Get or assign emoji based on signature pattern
    fn get_or_assign_emoji(&mut self, signature: &str) -> String {
        if let Some(emoji) = self.emoji_assignments.get(signature) {
            return emoji.clone();
        }
        
        let emoji = self.generate_emoji_for_signature(signature);
        self.emoji_assignments.insert(signature.to_string(), emoji.clone());
        emoji
    }

    /// Generate emoji based on signature characteristics
    fn generate_emoji_for_signature(&self, signature: &str) -> String {
        match signature {
            s if s.contains("pub_fn") => "🔧".to_string(),
            s if s.contains("pub_struct") => "🏗️".to_string(),
            s if s.contains("pub_enum") => "🎯".to_string(),
            s if s.contains("impl_block") => "⚙️".to_string(),
            s if s.contains("trait_def") => "🎭".to_string(),
            s if s.contains("macro_def") => "🪄".to_string(),
            s if s.contains("derive_attr") => "✨".to_string(),
            s if s.contains("prelude") => "🌟".to_string(),
            s if s.contains("use_stmt") => "📦".to_string(),
            _ => "🔍".to_string(),
        }
    }

    /// Reassign primes based on frequency (most common gets 2)
    pub fn optimize_prime_assignments(&mut self) {
        // Sort signatures by frequency (descending)
        let mut freq_sorted: Vec<_> = self.frequency_map.iter().collect();
        freq_sorted.sort_by(|a, b| b.1.cmp(a.1));
        
        // Reassign primes starting with 2 for most common
        self.prime_assignments.clear();
        let mut prime = 2u64;
        
        for (signature, _frequency) in freq_sorted {
            self.prime_assignments.insert(signature.clone(), prime);
            prime = self.next_prime_number(prime);
        }
        
        self.next_prime = prime;
    }

    /// Calculate signature compression ratio
    pub fn calculate_compression_ratio(&self) -> f64 {
        if self.frequency_map.is_empty() {
            return 1.0;
        }
        
        let total_signatures = self.frequency_map.len();
        let total_frequency: u64 = self.frequency_map.values().sum();
        
        // Most common signature gets prime 2 (50% probability)
        let most_common_freq = self.frequency_map.values().max().unwrap_or(&1);
        let compression_ratio = (*most_common_freq as f64) / (total_frequency as f64);
        
        compression_ratio
    }

    /// Generate signature statistics
    pub fn generate_statistics(&self) -> SignatureStats {
        let total_signatures = self.frequency_map.len();
        let total_frequency: u64 = self.frequency_map.values().sum();
        let most_common_freq = self.frequency_map.values().max().copied().unwrap_or(0);
        let least_common_freq = self.frequency_map.values().min().copied().unwrap_or(0);
        
        // Find most and least common signatures
        let most_common = self.frequency_map.iter()
            .max_by_key(|(_, &freq)| freq)
            .map(|(sig, _)| sig.clone())
            .unwrap_or_default();
            
        let least_common = self.frequency_map.iter()
            .min_by_key(|(_, &freq)| freq)
            .map(|(sig, _)| sig.clone())
            .unwrap_or_default();

        SignatureStats {
            total_signatures,
            total_frequency,
            most_common_signature: most_common,
            most_common_frequency: most_common_freq,
            least_common_signature: least_common,
            least_common_frequency: least_common_freq,
            compression_ratio: self.calculate_compression_ratio(),
            prime_range: (2, self.next_prime - 1),
        }
    }

    /// Find next prime number
    fn next_prime_number(&self, n: u64) -> u64 {
        let mut candidate = n + 1;
        while !self.is_prime(candidate) {
            candidate += 1;
        }
        candidate
    }

    /// Check if number is prime
    fn is_prime(&self, n: u64) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        
        let sqrt_n = (n as f64).sqrt() as u64;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

/// Statistics about signature compression
#[derive(Debug)]
pub struct SignatureStats {
    pub total_signatures: usize,
    pub total_frequency: u64,
    pub most_common_signature: String,
    pub most_common_frequency: u64,
    pub least_common_signature: String,
    pub least_common_frequency: u64,
    pub compression_ratio: f64,
    pub prime_range: (u64, u64),
}

/// Signature lookup table for fast access
pub struct SignatureLookup {
    pub prime_to_signature: HashMap<u64, String>,
    pub emoji_to_signature: HashMap<String, String>,
    pub signature_to_bindings: HashMap<String, Vec<String>>,
}

impl SignatureLookup {
    pub fn from_compressor(compressor: &SignatureCompressor) -> Self {
        let mut prime_to_signature = HashMap::new();
        let mut emoji_to_signature = HashMap::new();
        let mut signature_to_bindings = HashMap::new();
        
        for (signature, &prime) in &compressor.prime_assignments {
            prime_to_signature.insert(prime, signature.clone());
        }
        
        for (signature, emoji) in &compressor.emoji_assignments {
            emoji_to_signature.insert(emoji.clone(), signature.clone());
        }
        
        // Build bindings lookup (would need to store this in compressor)
        for signature in compressor.frequency_map.keys() {
            let bindings: Vec<String> = signature.split('|').map(|s| s.to_string()).collect();
            signature_to_bindings.insert(signature.clone(), bindings);
        }
        
        Self {
            prime_to_signature,
            emoji_to_signature,
            signature_to_bindings,
        }
    }
    
    /// Decode signature from prime
    pub fn decode_from_prime(&self, prime: u64) -> Option<Vec<String>> {
        self.prime_to_signature.get(&prime)
            .and_then(|sig| self.signature_to_bindings.get(sig))
            .cloned()
    }
    
    /// Decode signature from emoji
    pub fn decode_from_emoji(&self, emoji: &str) -> Option<Vec<String>> {
        self.emoji_to_signature.get(emoji)
            .and_then(|sig| self.signature_to_bindings.get(sig))
            .cloned()
    }
}
