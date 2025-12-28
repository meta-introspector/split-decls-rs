macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! reveal_with_filemanager1 {
    () => {
        deps!();
        fn reveal_with_filemanager1 (path : & Path , connection : & Connection) -> Result < () , OpenError > { let uri = path_to_uri (path) ? ; let proxy = FileManager1Proxy :: new (connection) . map_err (dbus_to_open_error) ? ; proxy . show_items (& [uri] , "") . map_err (dbus_to_open_error) }
    };
}

reveal_with_filemanager1!();