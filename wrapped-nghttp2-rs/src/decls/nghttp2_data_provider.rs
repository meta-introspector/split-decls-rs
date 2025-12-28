macro_rules! nghttp2_data_provider {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " This struct represents the data source and the way to read a chunk"] # [doc = " of data from it."] # [repr (C)] # [derive (Copy , Clone)] pub struct nghttp2_data_provider { # [doc = " The data source."] pub source : nghttp2_data_source , # [doc = " The callback function to read a chunk of data from the |source|."] pub read_callback : nghttp2_data_source_read_callback , }
    };
}

nghttp2_data_provider!();