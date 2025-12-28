macro_rules! deps {
    () => {
        Operation!();
        Context!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " Represents a thread blocked on a specific channel operation."] pub (crate) struct Entry { # [doc = " The operation."] pub (crate) oper : Operation , # [doc = " Optional packet."] pub (crate) packet : * mut () , # [doc = " Context associated with the thread owning this operation."] pub (crate) cx : Context , }
    };
}

Entry!()