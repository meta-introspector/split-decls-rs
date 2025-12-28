macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl From < (DefWithBodyId , LabelId) > for Label { fn from ((parent , label_id) : (DefWithBodyId , LabelId)) -> Self { Label { parent , label_id } } }
    };
}

impl_40!();