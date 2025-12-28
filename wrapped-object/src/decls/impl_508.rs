macro_rules! deps {
    () => {
        NodeIterator!();
        Result!();
        ExportsTrieIterator!();
        ExportSymbol!();
    };
}

macro_rules! impl_508 {
    () => {
        deps!();
        impl < 'data > ExportsTrieIterator < 'data > { pub (super) fn new (data : & 'data [u8]) -> Self { ExportsTrieIterator { node_iter : NodeIterator :: new (data) , } } # [doc = " Returns the next exported symbol, if any."] fn next (& mut self) -> Result < Option < ExportSymbol < 'data > > > { for node in & mut self . node_iter { if let Some (export_symbol) = node ? { return Ok (Some (export_symbol)) ; } } Ok (None) } }
    };
}

impl_508!();