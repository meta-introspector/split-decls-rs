macro_rules! deps {
    () => {
        PFUNCTION_TABLE_ACCESS_ROUTINE64!();
        BOOL!();
        SYMBOL_INFOW!();
        STACKFRAME_EX!();
        PENUMLOADED_MODULES_CALLBACKW64!();
        PTRANSLATE_ADDRESS_ROUTINE64!();
        HANDLE!();
        PCWSTR!();
        PREAD_PROCESS_MEMORY_ROUTINE64!();
        PGET_MODULE_BASE_ROUTINE64!();
        Symbol!();
        IMAGEHLP_LINEW64!();
        PWSTR!();
        STACKFRAME64!();
    };
}

macro_rules! macro_48 {
    () => {
        deps!();
        dbghelp ! { extern "system" { fn SymGetOptions () -> u32 ; fn SymSetOptions (options : u32) -> u32 ; fn SymInitializeW (handle : HANDLE , path : PCWSTR , invade : BOOL) -> BOOL ; fn SymGetSearchPathW (hprocess : HANDLE , searchpatha : PWSTR , searchpathlength : u32) -> BOOL ; fn SymSetSearchPathW (hprocess : HANDLE , searchpatha : PCWSTR) -> BOOL ; fn EnumerateLoadedModulesW64 (hprocess : HANDLE , enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64 , usercontext : * const c_void) -> BOOL ; fn StackWalk64 (MachineType : u32 , hProcess : HANDLE , hThread : HANDLE , StackFrame : * mut STACKFRAME64 , ContextRecord : * mut c_void , ReadMemoryRoutine : PREAD_PROCESS_MEMORY_ROUTINE64 , FunctionTableAccessRoutine : PFUNCTION_TABLE_ACCESS_ROUTINE64 , GetModuleBaseRoutine : PGET_MODULE_BASE_ROUTINE64 , TranslateAddress : PTRANSLATE_ADDRESS_ROUTINE64) -> BOOL ; fn SymFunctionTableAccess64 (hProcess : HANDLE , AddrBase : u64) -> * mut c_void ; fn SymGetModuleBase64 (hProcess : HANDLE , AddrBase : u64) -> u64 ; fn SymFromAddrW (hProcess : HANDLE , Address : u64 , Displacement : * mut u64 , Symbol : * mut SYMBOL_INFOW) -> BOOL ; fn SymGetLineFromAddrW64 (hProcess : HANDLE , dwAddr : u64 , pdwDisplacement : * mut u32 , Line : * mut IMAGEHLP_LINEW64) -> BOOL ; fn StackWalkEx (MachineType : u32 , hProcess : HANDLE , hThread : HANDLE , StackFrame : * mut STACKFRAME_EX , ContextRecord : * mut c_void , ReadMemoryRoutine : PREAD_PROCESS_MEMORY_ROUTINE64 , FunctionTableAccessRoutine : PFUNCTION_TABLE_ACCESS_ROUTINE64 , GetModuleBaseRoutine : PGET_MODULE_BASE_ROUTINE64 , TranslateAddress : PTRANSLATE_ADDRESS_ROUTINE64 , Flags : u32) -> BOOL ; fn SymFromInlineContextW (hProcess : HANDLE , Address : u64 , InlineContext : u32 , Displacement : * mut u64 , Symbol : * mut SYMBOL_INFOW) -> BOOL ; fn SymGetLineFromInlineContextW (hProcess : HANDLE , dwAddr : u64 , InlineContext : u32 , qwModuleBaseAddress : u64 , pdwDisplacement : * mut u32 , Line : * mut IMAGEHLP_LINEW64) -> BOOL ; fn SymAddrIncludeInlineTrace (hProcess : HANDLE , Address : u64) -> u32 ; fn SymQueryInlineTrace (hProcess : HANDLE , StartAddress : u64 , StartContext : u32 , StartRetAddress : u64 , CurAddress : u64 , CurContext : * mut u32 , CurFrameIndex : * mut u32) -> BOOL ; } }
    };
}

macro_48!();