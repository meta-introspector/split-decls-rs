macro_rules! ReferenceData {
    () => {
        # [derive (Debug)] pub struct ReferenceData { pub range : FileRange , pub is_definition : bool , }
    };
}

ReferenceData!()