macro_rules! Symbol {
    () => {
        # [doc = " Information from a symbol table entry."] pub struct Symbol < 'a > { name : & 'a str , address : u64 , }
    };
}

Symbol!();