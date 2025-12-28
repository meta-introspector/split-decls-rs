macro_rules! Column {
    () => {
        # [derive (Default)] struct Column { offset : usize , width : usize , }
    };
}

Column!()