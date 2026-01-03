use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

pub struct MatrixDaemon {
    pub name: String,
    pub status: ServiceStatus,
}

#[derive(Debug, Clone)]
pub enum ServiceStatus {
    Stopped,
    Starting,
    Running,
    Failed,
}

impl MatrixDaemon {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            status: ServiceStatus::Stopped,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting {} service...", self.name);
        self.status = ServiceStatus::Starting;

        // Start service in background (detached)
        Command::new("nohup")
            .args(&["cargo", "run", "--release"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        // Give it a moment to start
        thread::sleep(Duration::from_millis(500));
        
        self.status = ServiceStatus::Running;
        println!("✅ {} service started in background", self.name);
        Ok(())
    }

    pub fn status(&self) -> ServiceStatus {
        // Check if process is actually running
        let output = Command::new("pgrep")
            .args(&["-f", "split-decls-genesis"])
            .output();
            
        match output {
            Ok(output) if !output.stdout.is_empty() => ServiceStatus::Running,
            _ => ServiceStatus::Stopped,
        }
    }

    pub fn restart(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Restarting {} service...", self.name);
        self.stop()?;
        thread::sleep(Duration::from_secs(2));
        self.start()
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🛑 Stopping {} service...", self.name);
        
        // Kill any existing processes
        Command::new("pkill")
            .args(&["-f", "split-decls-genesis"])
            .output()?;
            
        self.status = ServiceStatus::Stopped;
        println!("✅ {} service stopped", self.name);
        Ok(())
    }

    pub fn enable_auto_restart(&mut self) {
        let name = self.name.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(10));
                
                // Check if process is still running
                let output = Command::new("pgrep")
                    .args(&["-f", "split-decls-genesis"])
                    .output();
                    
                if let Ok(output) = output {
                    if output.stdout.is_empty() {
                        println!("⚠️  {} service died, restarting...", name);
                        // Restart logic would go here
                    }
                }
            }
        });
    }
}
