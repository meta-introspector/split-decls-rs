macro_rules! EXCLUDED_NAMESPACES {
    () => {
        const EXCLUDED_NAMESPACES : & [& str] = & ["Windows.Foundation" , "Windows.Win32.Foundation"] ;
    };
}

EXCLUDED_NAMESPACES!()