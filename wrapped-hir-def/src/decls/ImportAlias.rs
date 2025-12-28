macro_rules! ImportAlias {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] pub enum ImportAlias { # [doc = " Unnamed alias, as in `use Foo as _;`"] Underscore , # [doc = " Named alias"] Alias (Name) , }
    };
}

ImportAlias!()