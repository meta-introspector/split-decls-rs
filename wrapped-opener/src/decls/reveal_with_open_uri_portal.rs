macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! reveal_with_open_uri_portal {
    () => {
        deps!();
        fn reveal_with_open_uri_portal (path : & Path , connection : & Connection) -> Result < () , OpenError > { let file = File :: open (path) . map_err (OpenError :: Io) ? ; let proxy = OpenURIProxy :: new (connection) . map_err (dbus_to_open_error) ? ; proxy . open_directory ("" , file . as_fd () . into () , HashMap :: new ()) . map_err (dbus_to_open_error) . map (| _ | ()) }
    };
}

reveal_with_open_uri_portal!()