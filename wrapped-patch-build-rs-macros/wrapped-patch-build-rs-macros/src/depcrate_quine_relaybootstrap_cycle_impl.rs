// Generated macro for bootstrap_cycle_impl (function)
macro_rules! Depcrate_quine_relaybootstrap_cycle_impl {
() => {
// Module: crate::quine_relay
// Provides: {"bootstrap_cycle_impl"}
// Dependencies: {}
# [decl (fn , name = "bootstrap_cycle_impl" , vis = "pub" , hash = "a5105c28")] pub fn bootstrap_cycle_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let _cycle_desc = input_str . value () ; quote ! { { println ! ("cargo:warning=🔄 Creating bootstrap cycle") ; let bootstrap_code = r###"
pub struct BootstrapCycle {
    pub stages: Vec<&'static str>,
    pub current: usize,
}

impl BootstrapCycle {
    pub fn new() -> Self {
        Self {
            stages: vec!["mes", "tinycc", "gcc", "llvm", "rustc"],
            current: 0,
        }
    }
    
    pub fn next_stage(&mut self) -> Option<&'static str> {
        if self.current < self.stages.len() {
            let stage = self.stages[self.current];
            self.current += 1;
            Some(stage)
        } else {
            None
        }
    }
    
    pub fn is_complete(&self) -> bool {
        self.current >= self.stages.len()
    }
}
            "### ; bootstrap_code . to_string () } } . into () }
};
}
