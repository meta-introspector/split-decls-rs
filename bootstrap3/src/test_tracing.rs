// Test file to demonstrate working function call tracing

use anyhow::Result;

// Working mkdeclfn macro with function call tracing
macro_rules! mkdeclfn_traced {
    ($(#[$attr:meta])* pub fn $fn_name:ident($($args:tt)*) -> $ret:ty { $($body:tt)* }) => {
        $(#[$attr])*
        pub fn $fn_name($($args)*) -> $ret {
            println!("🔧 Calling function: {}", stringify!($fn_name));
            { $($body)* }
        }
    };
    ($(#[$attr:meta])* fn $fn_name:ident($($args:tt)*) -> $ret:ty { $($body:tt)* }) => {
        $(#[$attr])*
        fn $fn_name($($args)*) -> $ret {
            println!("🔧 Calling function: {}", stringify!($fn_name));
            { $($body)* }
        }
    };
}

// Test function using the tracing macro
mkdeclfn_traced! {
    pub fn test_traced_function(name: &str) -> Result<String> {
        println!("Inside function with name: {}", name);
        Ok(format!("Hello, {}!", name))
    }
}

pub fn test_tracing() -> Result<()> {
    println!("🧪 Testing enhanced macro tracing...");
    
    let result = test_traced_function("Bootstrap3")?;
    println!("📤 Result: {}", result);
    
    Ok(())
}
