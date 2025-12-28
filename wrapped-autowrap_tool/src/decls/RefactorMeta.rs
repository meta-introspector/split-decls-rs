macro_rules! RefactorMeta {
    () => {
        # [derive (Deserialize)] pub struct RefactorMeta { pub name : String , pub output_format : String , }
    };
}

RefactorMeta!()