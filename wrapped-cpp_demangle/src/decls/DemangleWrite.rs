macro_rules! deps {
    () => {
        DemangleNodeType!();
    };
}

macro_rules! DemangleWrite {
    () => {
        deps!();
        # [doc = " Sink for demangled text that reports syntactic structure."] pub trait DemangleWrite { # [doc = " Called when we are entering the scope of some AST node."] fn push_demangle_node (& mut self , _ : DemangleNodeType) { } # [doc = " Same as `fmt::Write::write_str`."] fn write_string (& mut self , s : & str) -> fmt :: Result ; # [doc = " Called when we are exiting the scope of some AST node for"] # [doc = " which `push_demangle_node` was called."] fn pop_demangle_node (& mut self) { } }
    };
}

DemangleWrite!()