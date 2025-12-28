macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! dbus_to_open_error {
    () => {
        deps!();
        fn dbus_to_open_error (error : zbus :: Error) -> OpenError { OpenError :: Io (io :: Error :: other (error)) }
    };
}

dbus_to_open_error!();