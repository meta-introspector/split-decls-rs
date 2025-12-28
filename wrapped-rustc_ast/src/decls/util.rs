macro_rules! util {
    () => {
        pub mod util { pub mod case ; pub mod classify ; pub mod comments ; pub mod literal ; pub mod parser ; pub mod unicode ; }
    };
}

util!();