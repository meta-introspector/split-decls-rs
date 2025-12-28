macro_rules! ModuleKind {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Encodable , Decodable)] pub enum ModuleKind { Regular , Allocator , }
    };
}

ModuleKind!();