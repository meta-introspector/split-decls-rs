macro_rules! Scope {
    () => {
        # [doc = " A scope to start threads within."] pub type Scope < 'scope , 'env > = std :: thread :: Scope < 'scope , 'env > ;
    };
}

Scope!();