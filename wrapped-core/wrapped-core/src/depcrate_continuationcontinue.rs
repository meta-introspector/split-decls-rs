// Generated macro for Continue (trait)
macro_rules! Depcrate_continuationContinue {
() => {
// Module: crate::continuation
// Provides: {"Continue"}
// Dependencies: {}
# [doc = " A generic trait for managing and continuing compiler execution."] pub trait Continue < S = CapturedState , R = Resolution > where S : Serialize + for < 'de > Deserialize < 'de > + Debug + Clone , R : Serialize + for < 'de > Deserialize < 'de > + Debug + Clone , { # [doc = " Pauses execution, captures the current state, and waits for a resolution."] fn continue_execution (& self , state : S) -> R ; # [doc = " A method to initialize the continuation mechanism."] fn initialize (& self) -> Result < () , Box < dyn std :: error :: Error > > ; }
};
}
