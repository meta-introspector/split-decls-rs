macro_rules! deps {
    () => {
        Writer!();
        Relocation!();
    };
}

macro_rules! RelocateWriter {
    () => {
        deps!();
        # [doc = " A `Writer` which also records relocations."] pub trait RelocateWriter { # [doc = " The type of the writer being used to write the section data."] type Writer : Writer ; # [doc = " Get the writer being used to write the section data."] fn writer (& self) -> & Self :: Writer ; # [doc = " Get the writer being used to write the section data."] fn writer_mut (& mut self) -> & mut Self :: Writer ; # [doc = " Record a relocation."] fn relocate (& mut self , relocation : Relocation) ; }
    };
}

RelocateWriter!()