macro_rules! deps {
    () => {
        ExportTarget!();
    };
}

macro_rules! impl_686 {
    () => {
        deps!();
        impl < 'data > ExportTarget < 'data > { # [doc = " Returns true if the target is an address."] pub fn is_address (& self) -> bool { match self { ExportTarget :: Address (_) => true , _ => false , } } # [doc = " Returns true if the export is forwarded to another DLL."] pub fn is_forward (& self) -> bool { ! self . is_address () } }
    };
}

impl_686!();