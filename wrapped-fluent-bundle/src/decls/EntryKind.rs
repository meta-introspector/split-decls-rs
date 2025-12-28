macro_rules! EntryKind {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq)] pub enum EntryKind { Message , Term , Function , }
    };
}

EntryKind!();