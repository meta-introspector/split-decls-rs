macro_rules! MergedObjectField {
    () => {
        # [derive (FromField)] pub struct MergedObjectField { pub ty : Type , }
    };
}

MergedObjectField!();