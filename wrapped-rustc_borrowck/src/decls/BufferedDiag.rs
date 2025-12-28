macro_rules! BufferedDiag {
    () => {
        enum BufferedDiag < 'infcx > { Error (Diag < 'infcx >) , NonError (Diag < 'infcx , () >) , }
    };
}

BufferedDiag!();