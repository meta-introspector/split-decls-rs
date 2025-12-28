macro_rules! DemangleState {
    () => {
        # [derive (Debug , Copy , Clone)] struct DemangleState { # [doc = " How deep in the demangling are we?"] pub recursion_level : u32 , }
    };
}

DemangleState!()