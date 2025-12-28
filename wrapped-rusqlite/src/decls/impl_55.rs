macro_rules! impl_55 {
    () => {
        impl Default for OpenFlags { # [inline] fn default () -> Self { Self :: SQLITE_OPEN_READ_WRITE | Self :: SQLITE_OPEN_CREATE | Self :: SQLITE_OPEN_NO_MUTEX | Self :: SQLITE_OPEN_URI } }
    };
}

impl_55!()