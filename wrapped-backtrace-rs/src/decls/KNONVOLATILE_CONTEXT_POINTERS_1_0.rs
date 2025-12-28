macro_rules! KNONVOLATILE_CONTEXT_POINTERS_1_0 {
    () => {
        # [repr (C)] # [cfg (any (target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub struct KNONVOLATILE_CONTEXT_POINTERS_1_0 { pub Rax : * mut u64 , pub Rcx : * mut u64 , pub Rdx : * mut u64 , pub Rbx : * mut u64 , pub Rsp : * mut u64 , pub Rbp : * mut u64 , pub Rsi : * mut u64 , pub Rdi : * mut u64 , pub R8 : * mut u64 , pub R9 : * mut u64 , pub R10 : * mut u64 , pub R11 : * mut u64 , pub R12 : * mut u64 , pub R13 : * mut u64 , pub R14 : * mut u64 , pub R15 : * mut u64 , }
    };
}

KNONVOLATILE_CONTEXT_POINTERS_1_0!()