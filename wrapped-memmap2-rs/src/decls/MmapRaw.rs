macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! MmapRaw {
    () => {
        deps!();
        # [doc = " A handle to a raw memory mapped buffer."] # [doc = ""] # [doc = " This struct never hands out references to its interior, only raw pointers."] # [doc = " This can be helpful when creating shared memory maps between untrusted processes."] # [doc = ""] # [doc = " For the safety concerns that arise when converting these raw pointers to references,"] # [doc = " see the [`Mmap`] safety documentation."] pub struct MmapRaw { inner : MmapInner , }
    };
}

MmapRaw!();