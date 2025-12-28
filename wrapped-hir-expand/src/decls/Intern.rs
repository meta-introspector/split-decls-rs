macro_rules! Intern {
    () => {
        pub trait Intern { type Database : ? Sized ; type ID ; fn intern (self , db : & Self :: Database) -> Self :: ID ; }
    };
}

Intern!()