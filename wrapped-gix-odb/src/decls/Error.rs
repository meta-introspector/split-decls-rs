macro_rules! Error {
    () => {
        # [doc = " Returned by [`resolve()`]"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Io (# [from] io :: Error) , # [error (transparent)] Realpath (# [from] gix_path :: realpath :: Error) , # [error (transparent)] Parse (# [from] parse :: Error) , # [error ("Alternates form a cycle: {} -> {}" , . 0 . iter () . map (| p | format ! ("'{}'" , p . display ())) . collect ::< Vec < _ >> () . join (" -> ") , . 0 . first () . expect ("more than one directories") . display ())] Cycle (Vec < PathBuf >) , }
    };
}

Error!();