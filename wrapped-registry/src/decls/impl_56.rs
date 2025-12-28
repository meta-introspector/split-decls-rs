macro_rules! deps {
    () => {
        Key!();
        Transaction!();
        OpenOptions!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'a > OpenOptions < 'a > { pub (crate) fn new (parent : & 'a Key) -> Self { Self { parent , access : 0 , create : false , transaction : None , options : REG_OPTION_NON_VOLATILE , } } # [doc = " Sets the option for read access."] pub fn read (& mut self) -> & mut Self { self . access |= KEY_READ ; self } # [doc = " Sets the option for write access."] pub fn write (& mut self) -> & mut Self { self . access |= KEY_WRITE ; self } # [doc = " Sets additional access rights."] pub fn access (& mut self , access : u32) -> & mut Self { self . access |= access ; self } # [doc = " Sets the option to create a new registry key, or open it if it already exists."] pub fn create (& mut self) -> & mut Self { self . create = true ; self } # [doc = " Associate the registry key with a transaction."] pub fn transaction (& mut self , transaction : & 'a Transaction) -> & mut Self { self . transaction = Some (transaction) ; self } # [doc = " Sets the option to create a volatile registry key that is not preserved when the system restarts."] pub fn volatile (& mut self) -> & mut Self { self . options |= REG_OPTION_VOLATILE ; self } # [doc = " Opens a registry key with the options provided by `self`."] pub fn open < T : AsRef < str > > (& self , path : T) -> Result < Key > { let mut handle = null_mut () ; let result = unsafe { if let Some (transaction) = self . transaction { if self . create { RegCreateKeyTransactedW (self . parent . 0 , pcwstr (path) . as_ptr () , 0 , null () , self . options , self . access , null () , & mut handle , null_mut () , transaction . 0 , null () ,) } else { RegOpenKeyTransactedW (self . parent . 0 , pcwstr (path) . as_ptr () , 0 , self . access , & mut handle , transaction . 0 , null () ,) } } else if self . create { RegCreateKeyExW (self . parent . 0 , pcwstr (path) . as_ptr () , 0 , null () , self . options , self . access , null () , & mut handle , null_mut () ,) } else { RegOpenKeyExW (self . parent . 0 , pcwstr (path) . as_ptr () , 0 , self . access , & mut handle ,) } } ; win32_error (result) . map (| _ | Key (handle)) } }
    };
}

impl_56!()