macro_rules! deps {
    () => {
        SelectedOperation!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl Drop for SelectedOperation < '_ > { fn drop (& mut self) { panic ! ("dropped `SelectedOperation` without completing the operation") ; } }
    };
}

impl_186!();