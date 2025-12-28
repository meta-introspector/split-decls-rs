macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ioctl_write_buf {
    () => {
        deps!();
        # [doc = " Generates a wrapper function for an ioctl that writes an array of elements to the kernel."] # [doc = ""] # [doc = " The arguments to this macro are:"] # [doc = ""] # [doc = " * The function name"] # [doc = " * The ioctl identifier"] # [doc = " * The ioctl sequence number"] # [doc = " * The data type passed by this ioctl"] # [doc = ""] # [doc = " The generated function has the following signature:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " pub unsafe fn FUNCTION_NAME(fd: libc::c_int, data: &[DATA_TYPE]) -> Result<libc::c_int>"] # [doc = " ```"] # [doc = ""] # [doc = " For a more in-depth explanation of ioctls, see [`::sys::ioctl`](sys/ioctl/index.html)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate nix;"] # [doc = " const SPI_IOC_MAGIC: u8 = b'k'; // Defined in linux/spi/spidev.h"] # [doc = " const SPI_IOC_TYPE_MESSAGE: u8 = 0;"] # [doc = " # pub struct spi_ioc_transfer(u64);"] # [doc = " ioctl_write_buf!(spi_transfer, SPI_IOC_MAGIC, SPI_IOC_TYPE_MESSAGE, spi_ioc_transfer);"] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! ioctl_write_buf { ($ (# [$ attr : meta]) * $ name : ident , $ ioty : expr , $ nr : expr , $ ty : ty) => ($ (# [$ attr]) * pub unsafe fn $ name (fd : $ crate :: libc :: c_int , data : & [$ ty]) -> $ crate :: Result <$ crate :: libc :: c_int > { unsafe { convert_ioctl_res ! ($ crate :: libc :: ioctl (fd , request_code_write ! ($ ioty , $ nr , :: std :: mem :: size_of_val (data)) as $ crate :: sys :: ioctl :: ioctl_num_type , data . as_ptr ())) } }) }
    };
}

ioctl_write_buf!()