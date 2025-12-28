macro_rules! CsvReportWriter {
    () => {
        struct CsvReportWriter < W : Write > { writer : Writer < W > , }
    };
}

CsvReportWriter!();