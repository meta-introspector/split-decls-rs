macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        # [doc = " Handling of InMemory object writing"] impl crate :: Repository { # [doc = " When writing objects, keep them in memory instead of writing them to disk."] # [doc = " This makes any change to the object database non-persisting, while keeping the view"] # [doc = " to the object database consistent for this instance."] pub fn with_object_memory (mut self) -> Self { self . objects . enable_object_memory () ; self } }
    };
}

impl_293!()