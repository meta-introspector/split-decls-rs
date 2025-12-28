macro_rules! deps {
    () => {
        LogChange!();
        RefLog!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl Default for LogChange { fn default () -> Self { LogChange { mode : RefLog :: AndReference , force_create_reflog : false , message : Default :: default () , } } }
    };
}

impl_49!();