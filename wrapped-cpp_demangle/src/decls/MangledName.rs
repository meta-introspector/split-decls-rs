macro_rules! deps {
    () => {
        CloneSuffix!();
        GlobalCtorDtor!();
        Type!();
        Encoding!();
    };
}

macro_rules! MangledName {
    () => {
        deps!();
        # [doc = " The root AST node, and starting production."] # [doc = ""] # [doc = " ```text"] # [doc = " <mangled-name> ::= _Z <encoding> [<clone-suffix>]*"] # [doc = "                ::= ___Z <encoding> <block_invoke>"] # [doc = "                ::= <type>"] # [doc = ""] # [doc = " <block_invoke> ::= _block_invoke"] # [doc = "                ::= _block_invoke<decimal-digit>+"] # [doc = "                ::= _block_invoke_<decimal-digit>+"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum MangledName { # [doc = " The encoding of the mangled symbol name."] Encoding (Encoding , Vec < CloneSuffix >) , # [doc = " The encoding of the mangled symbol name."] BlockInvoke (Encoding , Option < isize >) , # [doc = " A top-level type. Technically not allowed by the standard, however in"] # [doc = " practice this can happen, and is tested for by libiberty."] Type (TypeHandle) , # [doc = " A global constructor or destructor. This is another de facto standard"] # [doc = " extension (I think originally from `g++`?) that is not actually part of"] # [doc = " the standard proper."] GlobalCtorDtor (GlobalCtorDtor) , }
    };
}

MangledName!()