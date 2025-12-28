macro_rules! FieldAccessError {
    () => {
        enum FieldAccessError { OutOfRange { field_count : usize } , }
    };
}

FieldAccessError!();