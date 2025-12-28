macro_rules! check_win_devices_and_illegal_characters {
    () => {
        fn check_win_devices_and_illegal_characters (input : & BStr) -> Option < component :: Error > { if is_win_device (input) { return Some (component :: Error :: WindowsReservedName) ; } if input . iter () . any (| b | * b < 0x20 || b":<>\"|?*" . contains (b)) { return Some (component :: Error :: WindowsIllegalCharacter) ; } if input . ends_with (b".") || input . ends_with (b" ") { return Some (component :: Error :: WindowsIllegalCharacter) ; } None }
    };
}

check_win_devices_and_illegal_characters!()