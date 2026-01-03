use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AstTrace {
    pub operation: String,
    pub node_type: String,
    pub location: String,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}

pub struct AstTracer {
    traces: Arc<Mutex<Vec<AstTrace>>>,
    enabled: bool,
}

impl AstTracer {
    pub fn new() -> Self {
        Self {
            traces: Arc::new(Mutex::new(Vec::new())),
            enabled: true,
        }
    }

    pub fn trace(&self, operation: &str, node_type: &str, location: &str, metadata: HashMap<String, String>) {
        if !self.enabled { return; }
        
        let trace = AstTrace {
            operation: operation.to_string(),
            node_type: node_type.to_string(),
            location: location.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            metadata,
        };

        if let Ok(mut traces) = self.traces.lock() {
            traces.push(trace);
        }
    }

    pub fn get_traces(&self) -> Vec<AstTrace> {
        self.traces.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        self.traces.lock().unwrap().clear();
    }
}

static GLOBAL_TRACER: OnceLock<AstTracer> = OnceLock::new();

pub fn init_tracer() -> &'static AstTracer {
    GLOBAL_TRACER.get_or_init(|| AstTracer::new())
}

pub fn trace_ast(operation: &str, node_type: &str, location: &str, metadata: HashMap<String, String>) {
    let tracer = init_tracer();
    tracer.trace(operation, node_type, location, metadata);
}
