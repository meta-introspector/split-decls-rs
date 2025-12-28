macro_rules! deps {
    () => {
        Symbol!();
        Table!();
    };
}

macro_rules! Symbols {
    () => {
        deps!();
        # [doc = " A symbol table."] pub type Symbols < 'data , const DYNAMIC : bool = false > = Table < Symbol < 'data , DYNAMIC > > ;
    };
}

Symbols!()