macro_rules! deps {
    () => {
        Retries!();
    };
}

macro_rules! error {
    () => {
        deps!();
        mod error { use std :: { fmt , path :: Path } ; use crate :: dir :: create :: Retries ; # [doc = " The error returned by [all()][super::all()]."] # [allow (missing_docs)] # [derive (Debug)] pub enum Error < 'a > { # [doc = " A failure we will probably recover from by trying again."] Intermediate { dir : & 'a Path , kind : std :: io :: ErrorKind } , # [doc = " A failure that ends the operation."] Permanent { dir : & 'a Path , err : std :: io :: Error , # [doc = " The retries left after running the operation"] retries_left : Retries , # [doc = " The original amount of retries to allow determining how many were actually used"] retries : Retries , } , } impl fmt :: Display for Error < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Intermediate { dir , kind } => write ! (f , "Intermediae failure creating {:?} with error: {:?}" , dir . display () , kind) , Error :: Permanent { err : _ , dir , retries_left , retries , } => write ! (f , "Permanently failing to create directory '{dir}' ({retries_left:?} of {retries:?})" , dir = dir . display () ,) , } } } impl std :: error :: Error for Error < '_ > { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { Error :: Permanent { err , .. } => Some (err) , _ => None , } } } }
    };
}

error!();