macro_rules! FileManager1 {
    () => {
        # [doc = " # D-Bus interface proxy for `org.freedesktop.FileManager1` interface."] # [zbus :: proxy (gen_async = false , interface = "org.freedesktop.FileManager1" , default_service = "org.freedesktop.FileManager1" , default_path = "/org/freedesktop/FileManager1")] trait FileManager1 { # [doc = " ShowItems method"] fn show_items (& self , uris : & [Url] , startup_id : & str) -> zbus :: Result < () > ; }
    };
}

FileManager1!();