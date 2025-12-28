macro_rules! deps {
    () => {
        Statistics!();
        BlamePathEntry!();
        BlameEntry!();
    };
}

macro_rules! Outcome {
    () => {
        deps!();
        # [doc = " The outcome of [`file()`](crate::file())."] # [derive (Debug , Default , Clone)] pub struct Outcome { # [doc = " One entry in sequential order, to associate a hunk in the blamed file with the source commit (and its lines)"] # [doc = " that introduced it."] pub entries : Vec < BlameEntry > , # [doc = " A buffer with the file content of the *Blamed File*, ready for tokenization."] pub blob : Vec < u8 > , # [doc = " Additional information about the amount of work performed to produce the blame."] pub statistics : Statistics , # [doc = " Contains a log of all changes that affected the outcome of this blame."] pub blame_path : Option < Vec < BlamePathEntry > > , }
    };
}

Outcome!()