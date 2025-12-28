macro_rules! RenameRule {
    () => {
        # [derive (Debug , Copy , Clone , FromMeta)] pub enum RenameRule { # [darling (rename = "lowercase")] Lower , # [darling (rename = "UPPERCASE")] Upper , # [darling (rename = "PascalCase")] Pascal , # [darling (rename = "camelCase")] Camel , # [darling (rename = "snake_case")] Snake , # [darling (rename = "SCREAMING_SNAKE_CASE")] ScreamingSnake , }
    };
}

RenameRule!()