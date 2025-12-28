macro_rules! deps {
    () => {
        SpawnProcessOnDemand!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl Drop for SpawnProcessOnDemand { fn drop (& mut self) { if let Some (mut child) = self . child . take () { child . kill () . ok () ; child . wait () . ok () ; } } }
    };
}

impl_37!()