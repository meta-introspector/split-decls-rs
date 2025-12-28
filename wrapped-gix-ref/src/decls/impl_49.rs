macro_rules! deps {
    () => {
        RefLog!();
        LogChange!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl Default for LogChange { fn default () -> Self { LogChange { mode : RefLog :: AndReference , force_create_reflog : false , message : Default :: default () , } } }
    };
}

impl_49!()