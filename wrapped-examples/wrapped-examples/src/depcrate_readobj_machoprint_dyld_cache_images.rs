// Generated macro for print_dyld_cache_images (function)
macro_rules! Depcrate_readobj_machoprint_dyld_cache_images {
() => {
// Module: crate::readobj::macho
// Provides: {"print_dyld_cache_images"}
// Dependencies: {}
pub (super) fn print_dyld_cache_images (p : & mut Printer < '_ > , cache : & DyldCache) { let endian = cache . endianness () ; let data = cache . data () ; for image in cache . images () { if p . options . file { let info = image . info () ; p . group ("DyldCacheImageInfo" , | p | { p . field_hex ("Address" , info . address . get (endian)) ; p . field_hex ("ModTime" , info . mod_time . get (endian)) ; p . field_hex ("Inode" , info . inode . get (endian)) ; p . field_string ("Path" , info . path_file_offset . get (endian) , info . path (endian , data) ,) ; p . field_hex ("Pad" , info . pad . get (endian)) ; }) ; } if let Some ((data , offset)) = image . image_data_and_offset () . print_err (p) { print_dyld_cache_image (p , data , offset , cache) ; p . blank () ; } } }
};
}
