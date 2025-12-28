macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! reveal_with_dbus {
    () => {
        deps!();
        pub (crate) fn reveal_with_dbus (path : & Path) -> Result < () , OpenError > { let connection = Connection :: session () . map_err (dbus_to_open_error) ? ; reveal_with_filemanager1 (path , & connection) . or_else (| _ | reveal_with_open_uri_portal (path , & connection)) }
    };
}

reveal_with_dbus!();