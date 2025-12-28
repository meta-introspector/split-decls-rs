macro_rules! deps {
    () => {
        AutoFinish!();
        AutoFinisher!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < T : AutoFinish > Drop for AutoFinisher < T > { fn drop (& mut self) { if let Some (writer) = self . 0 . take () { writer . finish_ignore_error () ; } } }
    };
}

impl_341!();