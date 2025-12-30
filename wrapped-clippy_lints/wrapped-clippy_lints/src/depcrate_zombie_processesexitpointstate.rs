// Generated macro for ExitPointState (enum)
macro_rules! Depcrate_zombie_processesExitPointState {
() => {
// Module: crate::zombie_processes
// Provides: {"ExitPointState"}
// Dependencies: {}
# [derive (Debug)] enum ExitPointState { # [doc = " Still walking up to the expression that initiated the visitor."] WalkUpTo (HirId) , # [doc = " We're inside of a control flow construct (e.g. `if`, `match`, `loop`)"] # [doc = " Within this, we shouldn't accept any `exit()` calls in here, but we can leave all of these"] # [doc = " constructs later and still continue looking for an `exit()` call afterwards. Example:"] # [doc = " ```ignore"] # [doc = " Command::new(\"\").spawn().unwrap();"] # [doc = ""] # [doc = " if true {                // depth=1"] # [doc = "     if true {            // depth=2"] # [doc = "         match () {       // depth=3"] # [doc = "             () => loop { // depth=4"] # [doc = ""] # [doc = "                 std::process::exit();"] # [doc = "                 ^^^^^^^^^^^^^^^^^^^^^ conditional exit call, ignored"] # [doc = ""] # [doc = "             }           // depth=3"] # [doc = "         }               // depth=2"] # [doc = "     }                   // depth=1"] # [doc = " }                       // depth=0"] # [doc = ""] # [doc = " std::process::exit();"] # [doc = " ^^^^^^^^^^^^^^^^^^^^^ this exit call is accepted because we're now unconditionally calling it"] # [doc = " ```"] # [doc = " We can only get into this state from `NoExit`."] InControlFlow { depth : u32 } , # [doc = " No exit call found yet, but looking for one."] NoExit , }
};
}
