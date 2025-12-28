macro_rules! DirectiveLocation {
    () => {
        # [derive (Debug , Copy , Clone , FromMeta , strum :: Display)] # [darling (rename_all = "PascalCase")] # [strum (serialize_all = "SCREAMING_SNAKE_CASE")] pub enum DirectiveLocation { Field , }
    };
}

DirectiveLocation!();