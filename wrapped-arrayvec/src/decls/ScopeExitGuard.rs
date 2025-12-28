macro_rules! ScopeExitGuard {
    () => {
        struct ScopeExitGuard < T , Data , F > where F : FnMut (& Data , & mut T) , { value : T , data : Data , f : F , }
    };
}

ScopeExitGuard!()