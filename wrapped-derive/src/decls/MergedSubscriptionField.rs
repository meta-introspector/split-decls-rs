macro_rules! MergedSubscriptionField {
    () => {
        # [derive (FromField)] pub struct MergedSubscriptionField { pub ty : Type , }
    };
}

MergedSubscriptionField!()