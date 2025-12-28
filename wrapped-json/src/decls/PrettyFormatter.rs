macro_rules! PrettyFormatter {
    () => {
        # [doc = " This structure pretty prints a JSON value to make it human readable."] # [derive (Clone , Debug)] pub struct PrettyFormatter < 'a > { current_indent : usize , has_value : bool , indent : & 'a [u8] , }
    };
}

PrettyFormatter!();