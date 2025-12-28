macro_rules! RunnableKindData {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Deserialize , Serialize)] # [serde (rename_all = "camelCase")] pub enum RunnableKindData { Check , Run , TestOne , }
    };
}

RunnableKindData!();