macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! NativeLibKind {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Encodable , Decodable , PrintAttribute)] # [derive (HashStable_Generic)] pub enum NativeLibKind { # [doc = " Static library (e.g. `libfoo.a` on Linux or `foo.lib` on Windows/MSVC)"] Static { # [doc = " Whether to bundle objects from static library into produced rlib"] bundle : Option < bool > , # [doc = " Whether to link static library without throwing any object files away"] whole_archive : Option < bool > , } , # [doc = " Dynamic library (e.g. `libfoo.so` on Linux)"] # [doc = " or an import library corresponding to a dynamic library (e.g. `foo.lib` on Windows/MSVC)."] Dylib { # [doc = " Whether the dynamic library will be linked only if it satisfies some undefined symbols"] as_needed : Option < bool > , } , # [doc = " Dynamic library (e.g. `foo.dll` on Windows) without a corresponding import library."] # [doc = " On Linux, it refers to a generated shared library stub."] RawDylib , # [doc = " A macOS-specific kind of dynamic libraries."] Framework { # [doc = " Whether the framework will be linked only if it satisfies some undefined symbols"] as_needed : Option < bool > , } , # [doc = " Argument which is passed to linker, relative order with libraries and other arguments"] # [doc = " is preserved"] LinkArg , # [doc = " Module imported from WebAssembly"] WasmImportModule , # [doc = " The library kind wasn't specified, `Dylib` is currently used as a default."] Unspecified , }
    };
}

NativeLibKind!()