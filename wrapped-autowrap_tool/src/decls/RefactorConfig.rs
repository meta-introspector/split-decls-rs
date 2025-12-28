macro_rules! deps {
    () => {
        ExportConfig!();
        RefactorMeta!();
        SplitRule!();
        DeclRefactoringConfig!();
    };
}

macro_rules! RefactorConfig {
    () => {
        deps!();
        # [derive (Deserialize)] pub struct RefactorConfig { pub refactor : RefactorMeta , # [serde (default)] pub splits : Vec < SplitRule > , pub export : ExportConfig , # [serde (default)] pub decl_refactoring : Option < DeclRefactoringConfig > , }
    };
}

RefactorConfig!();